actors Documentation
====================

Update this documentation by checking out the source branch in a separate
directory.

Then:

```bash
cd <YOUR SOURCE DIR>
cargo doc
cd <YOUR GH-PAGES_DIR>
cp -r <YOUR SOURCE DIR>/target/doc .
```

Now commit the resut and push it to origin gh-pages branch.