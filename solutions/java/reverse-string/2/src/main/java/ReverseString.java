class ReverseString {

    String reverse(String inputString) {
        char[] charStr = inputString.toCharArray();
        int len = charStr.length;
        for(int i=0; i<len/2; i++) {
        	char temp = charStr[i];
        	charStr[i] = charStr[len-i-1];
        	charStr[len-i-1] = temp;
        }
		return String.copyValueOf(charStr);
    }
  
}
