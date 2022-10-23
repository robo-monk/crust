./compile.sh

cd crust-ui
pnpm build
cd ..


# git add crust-ui/ crust-ui/crust/* -f
git add crust-ui/dist -f
git commit -m 'gh pages build'
git subtree push --prefix crust-ui/dist origin gh-pages
