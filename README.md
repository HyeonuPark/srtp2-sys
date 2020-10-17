srtp2
=========

![docs.rs](https://docs.rs/srtp2-sys/badge.svg)

Rust binding for libsrtp 2.3.0

Original repository: https://github.com/cisco/libsrtp

# Windows

Building from source is not supported on the MSVC target.
But you can install the library using [`vcpkg`](https://github.com/microsoft/vcpkg)
and link to it.

```
vcpkg install libsrtp --triplet x64-windows-static-md
```

Installed libsrtp version is not checked with vcpkg.

# Features

## `build`

Build the libsrtp from the source.
If this feature is not active, this crate tries to find
system wide installation using `pkg-config`.

You can pass environment variable `SRTP2_SYS_DEBUG_LOGGING` and optionally
`SRTP2_SYS_DEBUG_LOG_FILE` to activate debug logging of the libsrtp itself.
Note that the cargo caches the build artifacts so you need to `cargo clean`
before passing those variables.

## `enable-openssl`

Enable libsrtp2 features which requires the openssl library,
including cryptography using gcm mode and 192 bits algorithms.

System wide installations tend not to be compiled with this options.
It's recommended to use this feature with the `build` feature.

## `build-openssl`

Activate the `enable-openssl` feature, and also build the openssl from the source.

In case if you don't want to rely on the system package manager.

## `skip-linking`

Only generates bindings and skip any linking process.
Useful if all you want is to generate documentation.

# License and Disclaimer

libSRTP is distributed under the following license, which is included
in the source code distribution. It is reproduced in the manual in
case you got the library from another source.

> Copyright (c) 2001-2017 Cisco Systems, Inc.  All rights reserved.
>
> Redistribution and use in source and binary forms, with or without
> modification, are permitted provided that the following conditions
> are met:
>
> - Redistributions of source code must retain the above copyright
>   notice, this list of conditions and the following disclaimer.
> - Redistributions in binary form must reproduce the above copyright
>   notice, this list of conditions and the following disclaimer in
>   the documentation and/or other materials provided with the distribution.
> - Neither the name of the Cisco Systems, Inc. nor the names of its
>   contributors may be used to endorse or promote products derived
>   from this software without specific prior written permission.
>
> THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS
> "AS IS" AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT
> LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS
> FOR A PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL THE
> COPYRIGHT HOLDERS OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT,
> INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES
> (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR
> SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION)
> HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT,
> STRICT LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE)
> ARISING IN ANY WAY OUT OF THE USE OF THIS SOFTWARE, EVEN IF ADVISED
> OF THE POSSIBILITY OF SUCH DAMAGE.

--------------------------------------------------------------------------------
