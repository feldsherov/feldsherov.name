FROM rustlang/rust:nightly

WORKDIR /feldsherov.name
COPY  . .

RUN cargo install --path .

CMD ["fname"]
