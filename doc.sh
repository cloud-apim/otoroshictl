cd ./documentation
npm install
npm run build
cd ..
rm -rf ./docs
mv ./documentation/build ./docs 