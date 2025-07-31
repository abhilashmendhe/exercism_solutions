class RaindropConverter {

    String convert(int number) {
        int three = number % 3;
        int five  = number % 5;
        int seven = number % 7;
        if(three==0 && five==0 && seven==0)
            return "PlingPlangPlong";
        else if(three==0 && five==0)
            return "PlingPlang";
        else if(three==0 && seven==0)
            return "PlingPlong";
        else if(five==0 && seven==0)
            return "PlangPlong";
        else if(seven==0)
            return "Plong";
        else if(five==0)
            return "Plang";
        else if (three ==0)
            return "Pling";
        else
            return Integer.toString(number);
    }

}
