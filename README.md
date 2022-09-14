# Wundernut vol 12
Rust app that tries to find the hidden image from a parchment.

## What  
There's a hidden text in this one `parchment.png` image and this app tries to find it.  

## How  
The workflow: 
* we check if the image contains any discrepancies.
* Create a new image where the hidden characters are visible.
* Try to get the correct characters using Tesseract.  
* Cipher the text using Caesar cipher.  

## How to use  
### Easy way:
Have Docker installed and run the following:    
```
docker build -t wundernut .
docker run wundernut
```
NOTE: The docker file is really simple and the image size is around 4gb. So remember to delete it.  

### Other way:  
Have rust installed (I'm using 1.65.nightly) and run the following:  
```
sudo apt-get install libleptonica-dev libtesseract-dev clang tesseract-ocr-eng
cargo run
``` 
## Issues  
It doesn't work correctly. Passing the created png to some better OCR will result in the correct decrypted text. So the result is bit funky, but the idea is there.   

## Improvements  
* The dockerfile could be optimized  
* Fix the OCR. The result isn't correct.  
* Remove hardcoded parts (Leptess resolution, caesar cipher rotations, etc.)  