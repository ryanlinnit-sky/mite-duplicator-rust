import time
import multiprocessing
import signal
import sys

import zmq

MESSAGE_COUNT = 10_000_000


def _socket(addr, socket_type=zmq.PUSH):
    context = zmq.Context()
    socket = context.socket(socket_type)
    socket.connect(addr)

    return socket


def producer(addr, message_delay=0.0):
    socket = _socket(addr)

    time_since_last_message = time.time()
    time_since_last_print = time.time()
    time_to_sleep = message_delay / 1000

    for num in range(MESSAGE_COUNT):
        while time.time() - time_since_last_message < message_delay:
            time.sleep(time_to_sleep)

        # print(f"[{addr}] sender message #{num}")
        # work_message = {"num": num}
        message = f"message #{num}"
        socket.send(message.encode("utf-8"))
        # socket.send_json(work_message)

        time_since_last_message = time.time()

        # if time.time() - time_since_last_print > 5:
        #     print(f"Produced {num} messages")
        #     time_since_last_print = time.time()


def consumer(addr):
    def cleanup(signum, frame):
        final_output_message = f"[{addr}] Total Received {messages_consumed} messages"
        if messages_consumed < MESSAGE_COUNT:
            loss_percent = (MESSAGE_COUNT - messages_consumed) / MESSAGE_COUNT * 100
            final_output_message += f" (loss of {MESSAGE_COUNT - messages_consumed} messages, percent: {loss_percent:.2f}%)"
        print(final_output_message)
        sys.exit(0)

    signal.signal(signal.SIGTERM, cleanup)

    messages_consumed = 0
    messages_consumed_delta = 0
    messages_consumed_last = 0

    consumer_receiver = _socket(addr, zmq.PULL)

    time_since_last = time.time()

    while True:
        _work = consumer_receiver.recv()
        messages_consumed += 1
        if time.time() - time_since_last > 1:
            messages_consumed_delta = messages_consumed - messages_consumed_last
            messages_consumed_last = messages_consumed

            print(
                f"[{addr}] Received {messages_consumed} messages ({messages_consumed_delta} messages/s)"
            )
            time_since_last = time.time()


if __name__ == "__main__":
    consumer_processes = []
    for i in range(1):
        addr = f"tcp://127.0.0.1:1450{i}"
        print(f"Starting consumer connecting to {addr}")
        con_proc = multiprocessing.Process(target=consumer, args=(addr,))
        con_proc.start()
        consumer_processes.append(con_proc)

    time.sleep(1)

    producer_processes = []
    for _ in range(1):
        # slight delay to avoid a huge spike in messages
        time.sleep(0.01)
        addr = "tcp://127.0.0.1:14302"
        print(f"Starting producer connecting to {addr}")
        prod_proc = multiprocessing.Process(target=producer, args=(addr, 0.00000))
        prod_proc.start()
        producer_processes.append(prod_proc)

    for proc in producer_processes:
        proc.join()

    time.sleep(1)

    # when all the producers have finished
    # send a stop message to the consumers
    for proc in consumer_processes:
        proc.terminate()
        proc.join()
