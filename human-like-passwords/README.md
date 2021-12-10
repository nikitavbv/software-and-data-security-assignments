# Part 1

Human-like passwords were generated the following way:
- 4% are truly random, length is [8; 16], alphanumeric and symbols ('!', '.', '?', '%')
- 4% contain words (chosen with weighted random according to frequency) and numbers
- 8% are top-100 passwords
- 84% are from common passwords list

Weak passwords hashes were generated with sha1 and 64-byte long salt.\
Strong passwords hashes were generated with argon2 and 64-byte long salt.

Top passwords list from: https://github.com/danielmiessler/SecLists

# Part 2

Using hashes from [this message](https://t.me/c/1664103404/842).

Weak hashes use md5 without salt.

My first attempt was to use rainbow table (because there is no salt and I just wanted to try), but it seems that
any rainbow table I could find on internet is from 10 years ago and download links are broken.

That's why my next attempt was to bruteforce using hashcat on a gpu cloud instance.

With `hashcat -a 3` I was able to recover around 64% in around 10 minutes. That's all passwords with length of [1;8].
That was really fast and explains why there are no rainbow tables on the internet - you can generate hundreds of
gigabytes of hashes in 10 minutes yourself.

Next, I tried to use a dictionary (top 1 million passwords from previous part). I was able to recover around 73% of
passwords in around 1 second.

The intersection between two sets of passwords is not complete, so it makes sense to use both methods. Bruteforce is
good for cracking short "trully random" passwords, dictionary is good for searching top passwords and combined.

Strong hashes use argon2.

There is no argon2 mode in hashcat, so I quickly wrote a simple tool to check against top passwords myself.
Bruteforcing is out of question here.

Of course my own bruteforcer is no way as efficient as tools like hashcat are. In fact, it does not even saturate CPU
fully.

Because strong hashes use salt, I have to check each hash separately.

I was able to crack only around a thousand of passwords after checking against top 10 passwords.
I don't think it is possible to try some combined or trully random passwords. Even with efficient implementation it
is going to take too long.

# Recommendations:
- Use hashing algorithms which are difficult to bruteforce, like argon2.
- Enforce passwords to be at least 8 or 10 characters long. I am not sure if enforcing using special symbols is worth
it. Users will just append symbols at the end of their password and it will make dictionary search just a little bit
more difficult.
- It would be a nice idea to prevent users from using passwords which are too easy to guess. Maybe check those at the
time of the signup against the list of top passwords, similarly to how haveibeenpwned does that.