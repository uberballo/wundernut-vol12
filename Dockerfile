FROM rust:1.63

RUN apt-get update && apt-get install -y \
 libleptonica-dev \
 libtesseract-dev\
 clang \
 tesseract-ocr-eng 
WORKDIR /usr/src/myapp
COPY . .

RUN cargo install --path .

CMD ["wundernut"]