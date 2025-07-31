class Scrabble {

    private String word;
    Scrabble(String word) {
        this.word = word;
    }

    int getScore() {
        int sum = 0;
        for(int i=0; i<word.length(); i++){
            char c = word.charAt(i);
            switch(c){
                case 'A':
                case 'E':
                case 'I':
                case 'O':
                case 'U':
                case 'L':
                case 'N':
                case 'R':
                case 'S':
                case 'T':
                case 'a':
                case 'e':
                case 'i':
                case 'o':
                case 'u':
                case 'l':
                case 'n':
                case 'r':
                case 's':
                case 't':
                    sum+=1;
                    break;
                case 'D':
                case 'G':
                case 'd':
                case 'g':
                    sum+=2;
                    break;
                case 'B':
                case 'C':
                case 'M':
                case 'P':
                case 'b':
                case 'c':
                case 'm':
                case 'p':
                    sum+=3;
                    break;
                case 'F':
                case 'H':
                case 'V':
                case 'W':
                case 'Y':
                case 'f':
                case 'h':
                case 'v':
                case 'w':
                case 'y':
                    sum+=4;
                    break;
                case 'K':
                case 'k':
                    sum+=5;
                    break;
                case 'J':
                case 'X':
                case 'j':
                case 'x':
                    sum+=8;
                    break;
                case 'Q':
                case 'Z':
                case 'q':
                case 'z':
                    sum+=10;
                    break;
            }            
        }
        return sum;
    }

}
