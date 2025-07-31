public class Twofer {
    public String twofer(String name) {
    	if(name==null)
			return "One for you, one for me.";
        int lenStr = name.length();
		if(lenStr==0)
			return "One for you, one for me.";
		else
			return "One for " + name + ", one for me.";
    }
}
