class Darts {
    int score(double xOfDart, double yOfDart) {
        double side = Math.sqrt(xOfDart * xOfDart + yOfDart * yOfDart);

        if(side > 10.0)
            return 0;
        else if(side <= 10.0 && side > 5)
            return 1;
        else if(side <= 5 && side > 1)
            return 5;
        return 10;
        
    }
}
