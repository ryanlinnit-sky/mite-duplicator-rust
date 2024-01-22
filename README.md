# mite duplicator rust

```bash
cargo run -- --message-socket=tcp://0.0.0.0:14302 tcp://0.0.0.0:14304 tcp://0.0.0.0:14305
```

## todo

[ ] change producer.py to ensure data integrity between prod/con - time and error report 

[ ] in `src/main.rs:run` defer the sending of message to a seperate thread/proc?
    share a `Vec`? (some form of data stream?)


# python test script

```
python3.11 -m venv venv
source venv/bin/activate

pip install -r requirements.txt # literally just zmq

python producer.py
```