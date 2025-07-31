import java.util.Random;

class CaptainsLog {

    private static final char[] PLANET_CLASSES = new char[]{'D', 'H', 'J', 'K', 'L', 'M', 'N', 'R', 'T', 'Y'};

    private Random random;

    CaptainsLog(Random random) {
        this.random = random;
    }

    char randomPlanetClass() {
        int planet_size = this.PLANET_CLASSES.length;
        int randIndex = this.random.nextInt(planet_size);
        return this.PLANET_CLASSES[randIndex];
    }

    String randomShipRegistryNumber() {
        int randNum = this.random.nextInt(9000) + 1000;
        return "NCC-"+randNum;
    }

    double randomStardate() {
        return 41000.0 + 1000*this.random.nextDouble();
    }
}
