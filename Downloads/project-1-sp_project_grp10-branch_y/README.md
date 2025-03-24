[![Review Assignment Due Date](https://classroom.github.com/assets/deadline-readme-button-22041afd0340ce965d47ae6ef1cefeee28c7c493a6346c4f15d667ab976d596c.svg)](https://classroom.github.com/a/7ZFsTk8-)


“I/We have done this assignment completely on our own. I/We have not copied it, nor
have I/we given my solution to anyone else. I/We understand that if we are involved in
plagiarism or cheating we will have to sign an official form that we have cheated and that
this form will be stored in my official university record. I/we also understand that I/we
will receive a grade of 0 for the involved assignment and my/our grade will be reduced
by one level (e.g., from A to B) for my/our offense, and that I/we will receive a grade of
“F” for the course in the case of egregious examples or any additional offense of any
kind.”

Abdul Mannan Kitab 

Yashpreet Khambay 


Doc:
 Assignment
https://blessed.rs/crates#section-cryptography


Task1:
Command Line Argument Parser for Rust: clap crate

Rand crate
Use arc , shared memory mutex, counter, atomic and ordering

BufWriter,Writer

Thiserror

Color-eyre

All unwrap error conveted
Add tracing and sleep to threads

References:
https://youtu.be/fD9ptABVQbI?si=AVokuELx1EPkvE-G
https://docs.rs/clap/latest/clap/
https://youtu.be/B_UZu-jBYgw?si=JQsIw51DHrFHwzOz
https://youtu.be/Ot3qCA3Iv_8?si=29wOLRtGVyFI-SWW , https://docs.rs/clap/latest/clap/_derive/_tutorial/index.html ->validation

https://doc.rust-lang.org/std/sync/atomic/index.html
https://doc.rust-lang.org/std/sync/atomic/enum.Ordering.html


https://github.com/HKarimiA/rust-generate-random-string
https://www.geeksforgeeks.org/ascii-table/
https://rust-lang-nursery.github.io/rust-cookbook/algorithms/randomness.html

How to Write Data to Files in Rust
https://doc.rust-lang.org/std/fs/struct.OpenOptions.html
https://doc.rust-lang.org/std/macro.writeln.html
https://doc.rust-lang.org/std/macro.write.html
https://stackoverflow.com/questions/73977425/how-can-i-output-data-from-a-vector-to-a-text-file

Thiserror crate , from trait
https://doc.rust-lang.org/std/convert/trait.From.html
https://doc.rust-lang.org/rust-by-example/error/multiple_error_types/boxing_errors.html
https://rust-lang-nursery.github.io/rust-cookbook/errors/handle.html#thiserror
https://mmapped.blog/posts/12-rust-error-handling

Color-eyre
https://docs.rs/color-eyre/latest/color_eyre/

Additions can consider:
https://users.rust-lang.org/t/how-to-create-bufreader---from-option-file-with-std-io-stdout-as-fallback-in-a-rust-way/12980
https://users.rust-lang.org/t/is-there-a-way-to-make-a-bufwriter-to-std-output-mutable-inside-a-closure/65201


Part 2:
Hashing

Bufreader
Hashing
https://github.com/RustCrypto/hashes
https://docs.rs/md-5/latest/md5/

Sender/receiver


COnsider mor elagorithms rom this: 
https://docs.rs/sha2/latest/sha2/
https://docs.rs/sha3/latest/sha3/
https://docs.rs/md-5/latest/md5/


Scrypt
https://github.com/rustcrypto/password-hashes

https://docs.rs/generic-array/latest/generic_array/struct.GenericArrayIter.html#method.as_slice

https://doc.rust-lang.org/rust-by-example/types/literals.html#literals

Return in closure, whats the significance? .send in seeparete thread, print by creating a vector, carete usage?

PHC string ? or seapte hash output
Crates
Space in password




Part 3:

https://users.rust-lang.org/t/read-variable-number-of-bytes-from-a-file/89179/7
https://doc.rust-lang.org/std/io/trait.Read.html#method.read_exact
https://doc.rust-lang.org/std/io/trait.Read.html#method.take
https://users.rust-lang.org/t/how-to-read-a-specified-number-of-bytes-from-a-file-into-vec/105109

https://doc.rust-lang.org/std/string/struct.String.html#method.from_utf8_lossy

https://passlib.readthedocs.io/en/stable/lib/passlib.hash.scrypt.html#:~:text=checksum%20%2D%20this%20is%20the%20base64,in%20a%2043%2Dcharacter%20checksum.

https://stackoverflow.com/questions/23985540/whats-the-is-maximum-length-of-scrypt-output
https://docs.rs/scrypt/0.11.0/scrypt/struct.Params.html
https://doc.rust-lang.org/std/slice/struct.ChunksExact.html



Commons:

Tracing: Tracing crate Rust program code walk through
https://stackoverflow.com/questions/70013172/how-to-use-the-tracing-library

diff <(sort dump-output.txt) <(sort 100-md5- dump.txt)
https://doc.rust-lang.org/rust-by-example/meta/doc.html

Tracing:
 Tracing 
Tracing crate Rust program code walk through








