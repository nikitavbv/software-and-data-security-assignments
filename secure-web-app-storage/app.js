const express = require('express');
const bodyParser = require('body-parser');
const {KeyManagementServiceClient} = require('@google-cloud/kms');
const {Storage} = require('@google-cloud/storage');
const uuid = require('uuid');
const { PassThrough } = require('stream');
const getRawBody = require('raw-body');

const app = express();
app.use(bodyParser.raw({ type: '*/*', limit : '100mb'}));
const port = 8080;

// Instantiates a client
const client = new KeyManagementServiceClient();

// Build the location name
const locationName = client.locationPath('nikitavbv', 'europe-central2');
const storage = new Storage();

app.get('/', (req, res) => {
    res.send('ok');
});

app.post('/images', async (req, res) => {
    const parts = [];

    for (let i = 0; i < req.body.length; i+=50000) {
        parts.push(req.body.slice(i, Math.min(i+50000, req.body.length)));
    }

    const encrypted = await Promise.all(parts.map(part => client.encrypt({
        name: 'projects/nikitavbv/locations/europe-central2/keyRings/secure-web-app/cryptoKeys/key1/cryptoKeyVersions/1',
        plaintext: part,
    })));
    const unwrapped = Buffer.from(JSON.stringify(encrypted.map(enc => enc[0].ciphertext.toString('base64'))), 'utf8');

    const fileName = uuid.v4();

    const target = storage.bucket('nikitavbv-secure-web-app').file(fileName);
    const passthroughStream = new PassThrough();
    passthroughStream.write(unwrapped);
    passthroughStream.end();
    await passthroughStream.pipe(target.createWriteStream());

    res.send(fileName);
});

app.get('/images/:fileId', async (req, res) => {
    const file = storage.bucket('nikitavbv-secure-web-app').file(req.params.fileId);

    const stream = file.createWriteStream();
    const body = await getRawBody(file.createReadStream());
    const unwrapped = JSON.parse(body);

    const decrypted = await Promise.all(unwrapped.map(part => client.decrypt({
        name: 'projects/nikitavbv/locations/europe-central2/keyRings/secure-web-app/cryptoKeys/key1',
        ciphertext: Buffer.from(part, 'base64'),
    })));
    const result = Buffer.concat(decrypted.map(enc => enc[0].plaintext));

    res.send(result);
});

app.listen(port, () => {
    console.log(`Example app listening at http://localhost:${port}`)
})