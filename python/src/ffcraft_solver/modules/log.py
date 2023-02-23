from datetime import datetime


class Loggers:
    def __init__(self):
        timestamp = self.get_timestamp()
        self.log = f"{timestamp} Log initialised."

    def add_log(self, other: str) -> str:
        timestamp = self.get_timestamp()
        self.log += '\n' + timestamp + ' ' + other
        print(self.log)
        return self.log

    # noinspection PyMethodMayBeStatic
    def get_timestamp(self) -> str:
        now = datetime.now()
        almost_timestamp = str(now.time())
        timestamp = almost_timestamp.rsplit('.')
        return f'[{timestamp[0]}]'


if __name__ == "__main__":
    loggers = Loggers()
    print(loggers.log)
