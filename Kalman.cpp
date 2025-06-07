import numpy as np

class Vector2D:
    def __init__(self, x, y):
        self.x = x
        self.y = y

class KalmanFilter2D:
    def __init__(self, initial_position, initial_velocity, process_noise, measurement_noise):
        self.state = np.array([initial_position.x, initial_position.y])
        self.velocity = np.array([initial_velocity.x, initial_velocity.y])
        self.error_covariance = np.array([[1, 0], [0, 1]])
        self.process_noise = process_noise
        self.measurement_noise = measurement_noise
        self.kalman_gain = np.zeros((2, 2))

    def predict(self, dt):
        self.state += self.velocity * dt
        self.error_covariance += self.process_noise * np.eye(2)

    def update(self, measurement):
        self.kalman_gain[0, 0] = self.error_covariance[0, 0] / (self.error_covariance[0, 0] + self.measurement_noise)
        self.kalman_gain[1, 1] = self.error_covariance[1, 1] / (self.error_covariance[1, 1] + self.measurement_noise)

        self.state += self.kalman_gain @ (np.array([measurement.x, measurement.y]) - self.state)
        self.error_covariance = (np.eye(2) - self.kalman_gain) @ self.error_covariance

    def get_state(self):
        return Vector2D(self.state[0], self.state[1])

def main():
    initial_position = Vector2D(0, 0)
    initial_velocity = Vector2D(1, 1)
    process_noise = 1e-3
    measurement_noise = 1e-2
    dt = 1.0

    kf = KalmanFilter2D(initial_position, initial_velocity, process_noise, measurement_noise)

    measurements = [Vector2D(1.1, 0.9), Vector2D(2.0, 2.1), Vector2D(3.0, 3.1), Vector2D(4.1, 4.0), Vector2D(5.0, 5.2)]

    for measurement in measurements:
        kf.predict(dt)
        kf.update(measurement)

        state = kf.get_state()
        print(f"Position estimée: ({state.x}, {state.y})")

if __name__ == "__main__":
    main()