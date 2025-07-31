public class ElonsToyCar {

    private int meters = 0;
    private int battery = 100;
    public static ElonsToyCar buy() {
        return new ElonsToyCar();
    }

    public String distanceDisplay() {
        return "Driven "+this.meters+" meters";
    }

    public String batteryDisplay() {
        if(this.battery>0)
            return "Battery at "+battery+"%";
    
        return "Battery empty";
    }

    public void drive() {
        if(this.battery > 0){
            this.meters += 20;
            this.battery -= 1;
        }
    }
}
