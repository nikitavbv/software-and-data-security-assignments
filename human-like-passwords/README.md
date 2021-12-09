Human-like passwords were generated the following way:
- 4% are truly random, length is [8; 16], alphanumeric and symbols ('!', '.', '?', '%')
- 4% contain words (chosen with weighted random according to frequency) and numbers
- 8% are top-100 passwords
- 84% are from common passwords list

Weak passwords hashes were generated with sha1 and 64-byte long salt.\
Strong passwords hashes were generated with argon2 and 64-byte long salt.

Top passwords list from: https://github.com/danielmiessler/SecLists
