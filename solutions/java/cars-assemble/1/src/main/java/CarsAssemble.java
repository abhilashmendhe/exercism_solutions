public class CarsAssemble {

    public double productionRatePerHour(int speed) {
        if (speed==0)
            return 0.0;
        double rate = speed * 221;
        if(speed >=1 && speed <5)
            return rate;
        else if(speed >=5 && speed < 9)
            return rate*0.9;
        else if(speed == 9)
            return rate*0.8;
        else
            return rate*0.77;
    }

    public int workingItemsPerMinute(int speed) {
        if (speed==0)
            return 0;
        double items = 221.0/60.0 * speed;
        if(speed >=1 && speed <5)
            return (int)items;
        else if(speed >= 5 && speed < 9)
            return (int)(items * 0.9);
        else if(speed==9)
            return (int)(items*.8);
        else
            return (int)(items*.77);
    }
}
