from .flitton_fib_rs import calculate_times, calculate_parameters


class NumpyInterface:
    
    def __init__(self):
        self.inventory = {}
    
    def calc_times(self, distance, traffic_grade):
        result = calculate_times(distance, traffic_grade)
        self.inventory["car time"] = result[0][0]
        self.inventory["truck time"] = result[1][0]
        
    def calc_parameters(self, car_time, truck_time):
        result = calculate_parameters(car_time, truck_time)
        self.inventory["distance"] = result[0][0]
        self.inventory["traffic grade"] = result[1][0]
