
Either
======

The Either enum.

Please read the `API documentation here`__

__ http://bluss.github.io/either/

|build_status|_ |crates|_

.. |build_status| image:: https://travis-ci.org/bluss/either.svg?branch=master
.. _build_status: https://travis-ci.org/bluss/either

.. |crates| image:: http://meritbadge.herokuapp.com/either
.. _crates: https://crates.io/crates/either

How to use with cargo::

    [dependencies]
    either = "0.1"


Recent Changes
--------------

- 0.1.2

  - Add macros `try_left!` and `try_right!`.

- 0.1.1

  - Implement Deref, DerefMut

- 0.1.0

  - Initial release
  - Support Iterator, Read, Write

License
-------

Dual-licensed to be compatible with the Rust project.

Licensed under the Apache License, Version 2.0
http://www.apache.org/licenses/LICENSE-2.0 or the MIT license
http://opensource.org/licenses/MIT, at your
option. This file may not be copied, modified, or distributed
except according to those terms.
