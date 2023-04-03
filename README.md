# Insignia


## What is it about?

Insignia is a program that takes a file as input (Currently it picks the file named `plaintext` from the source directory) and encrypt and sign it. This is done so that any receiver of the file and signature and public key, can verify that the file was signed by the authentic sender and is not a potentially malicious file.

## How it works?

Insignia takes a file named `plaintext` and then encrypts it usign 4096 Bit RSA Key. And then it is hashed using Ed25519 Private Key which results in the signature of the encrypted file, which can then be verified for integrity and authenticity of the file received by the receiver using Ed25519 Public key from The Ed25519 Keypair received along with the file.

## Purpose?

It is important where there is a need for secure communication of file transfer, like exchanging of sensitive information or transmission of confidential data over the internet. By signing the encrypted file the sender can be assured that the recipient will be able to verify that the file received has not been altered or tampered with during transit, and that it was indeed sent by the sender who holds the private key.
