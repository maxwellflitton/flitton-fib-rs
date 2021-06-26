from typing import Union, List, Optional

from .flitton_fib_rs import fibonacci_number, fibonacci_numbers


class FlittonFibNumberAdapter:

    def __init__(self, number_input: Union[int, List[int]]) -> None:
        self.number_input: Union[int, List[int]] = number_input
        self.success: bool = False
        self.result: Optional[Union[int, List[int]]] = None
        self.error_message: Optional[str] = None
        self._process_input()

    def _process_input(self) -> None:
        if isinstance(self.number_input, int):
            self.result = fibonacci_number(n=self.number_input)
            self.success = True
        elif isinstance(self.number_input, list):
            self.result = fibonacci_numbers(numbers=self.number_input)
            self.success = True
        else:
            self.error_message = "input needs to be a list of ints or an int"
