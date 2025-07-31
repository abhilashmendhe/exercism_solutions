class ProductionRemoteControlCar implements RemoteControlCar, Comparable<ProductionRemoteControlCar>{

    private int distance;
    private int numberOfVictories;
    public void drive() {
        this.distance += 10;
    }

    public int getDistanceTravelled() {
        return this.distance;
    }

    @Override
    public int compareTo(ProductionRemoteControlCar prcc){
        
        return this.numberOfVictories - prcc.getNumberOfVictories();
    }
    public int getNumberOfVictories() {
        return numberOfVictories;
    }

    public void setNumberOfVictories(int numberOfVictories) {
        this.numberOfVictories = numberOfVictories;
    }
}
