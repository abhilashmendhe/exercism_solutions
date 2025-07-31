import java.math.BigInteger;

class Grains {

    BigInteger grainsOnSquare(final int square) {
        // throw new UnsupportedOperationException("Delete this statement and write your own implementation.");
        if(square < 1 || square > 64)
            throw new IllegalArgumentException("square must be between 1 and 64");
        BigInteger val = BigInteger.valueOf(2);
        BigInteger nval = val.pow(square-1);
        return nval;
    }

    BigInteger grainsOnBoard() {
        return BigInteger.valueOf(2).pow(64).subtract(BigInteger.valueOf(1));
    }

}
