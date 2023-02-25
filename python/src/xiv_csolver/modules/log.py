from datetime import datetime
import dearpygui.dearpygui as dpg


class Loggers:
    def __init__(self, log=None):
        timestamp = self.get_timestamp()
        if not log:
            self.log = f"{timestamp} Log initialised."
        else:
            self.log = log

    def add_log(self, other: str) -> str:
        timestamp = self.get_timestamp()
        self.log = timestamp + ' ' + other + '\n' + self.log
        dpg.set_value('log_window', self.log)
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
