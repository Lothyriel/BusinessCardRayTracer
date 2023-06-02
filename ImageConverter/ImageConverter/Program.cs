using ImageConverter;

var path = "..//..//..//..//..//card.ppm";

var file = File.ReadAllText(path);

Conversion.ToPng(file, path);