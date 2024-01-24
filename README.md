# mite duplicator rust

```bash
# build with optimizations
cargo run --release -- --message-socket=tcp://0.0.0.0:14302 tcp://0.0.0.0:14500

# cargo build --release
```

## todo

[ ] include time metrics in `producer.py`

[ ] add a couple more debug messages in `main.rs`


# python test script

```
python3.11 -m venv venv
source venv/bin/activate

pip install -r requirements.txt # literally just zmq

python producer.py
```