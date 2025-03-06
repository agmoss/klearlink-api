# Release

1. `git cliff --bump [major|minor|patch]`
2. `cargo bump [FLAGS] [<version> | major | minor | patch]`
3. `git add -A`
4. `git commit -m "chore: Release ${version}"`
5. `git tag "${version}"`