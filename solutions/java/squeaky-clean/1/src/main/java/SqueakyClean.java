class SqueakyClean {
    static String clean(String identifier) {
        StringBuilder str = new StringBuilder();
		char []bytes = identifier.toCharArray();
		int i=0;
		while(i<bytes.length) {
			int intCharVal = (int)bytes[i];
			if(Character.isAlphabetic(bytes[i]))
				if(intCharVal < 945 || intCharVal > 969)
					str.append(bytes[i]);
			if(Character.isISOControl(bytes[i]))
				str.append("CTRL");
            if(intCharVal==32)
				str.append('_');    
            if(bytes[i]=='-') {
				char frontC = bytes[i+1];
                if(Character.isAlphabetic(frontC))
				str.append(Character.toUpperCase(frontC));
				i++;
			}
			i++;
		}
		return str.toString();
    }
}
