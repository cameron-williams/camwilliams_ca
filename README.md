camwilliams_ca

This repo holds the new version of my personal site (camwilliams.ca).

Figured it would be a good idea to build a nicer more modern version of my old website repo.

Now including:

- webpack + babel
- bootstrap
- multiple different back ends

It is currently a work in progress.


To run this repo locally you will need to have the following:
- cargo+rust (min 1.44.1)
- NPM (min 6.14.5)
- yarn (min 1.22.4)
- GNU make (min 4.2.1)

```sh
$ rustc --version && cargo --version && npm --version && && yarn --version && make --version
rustc 1.44.1 (c7087fe00 2020-06-17)
cargo 1.44.1 (88ba85757 2020-06-11)
6.14.5
yarn install v1.22.4
GNU Make 4.2.1

$ git clone git@github.com:cameron-williams/camwilliams_ca.git

$ cd camwilliams_ca

$ (cd frontend && npm install)

$ make run
=> http://127.0.0.1:3000/
```