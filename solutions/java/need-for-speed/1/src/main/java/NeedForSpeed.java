class NeedForSpeed {
    private int speed;
    private int batteryDrain;
    private int battery = 100;
    private int distance;
    NeedForSpeed(int speed, int batteryDrain) {
        this.speed = speed;
        this.batteryDrain = batteryDrain;
    }

    public boolean batteryDrained() {
        if(this.battery <= 0)
            return true;
        return false;
    }

    public int distanceDriven() {
        return this.distance;
    }

    public void drive() {
        if(this.battery > 0 && this.battery >= this.batteryDrain){
            this.battery -= this.batteryDrain;
            this.distance += this.speed;
        }
    }

    public static NeedForSpeed nitro() {
        return new NeedForSpeed(50, 4);
    }
}

class RaceTrack {
    private int distance;
    RaceTrack(int distance) {
        this.distance = distance;
    }

    public boolean tryFinishTrack(NeedForSpeed car) {
        while(car.distanceDriven() < this.distance && !car.batteryDrained()) {
    		car.drive();
    	}
    	return car.distanceDriven()==this.distance?true:false;
    }
}
