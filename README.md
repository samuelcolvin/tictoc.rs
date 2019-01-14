# toc toc in rust

To test

    rustc toc.rs && ./toc 123456

and

    rustc tic.rs && ./tic


To build for production

    make

To use in bash, include `tic` and `toc` in `PATH` somehow, then add to your `.bashrc` file:

```bash
export PS1='${PSPREFIX}\W $? `toc $TIC` ➤  '
_before () {
  export TIC=`tic`
}
trap '_before' DEBUG
```
