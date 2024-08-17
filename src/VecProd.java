import java.io.BufferedReader;
import java.io.InputStreamReader;
import java.io.IOException;

public class VecProd {
	static BufferedReader reader = new BufferedReader(new InputStreamReader(System.in));

	// Calculates the largest angle in radians between these 3 vectors
	public static void main(String[] args) throws IOException {
		double[] a = genVec("a");
		double[] b = genVec("b");
		double[] c = genVec("c");

		double ab = getScalarProduct(a, b);
		double bc = getScalarProduct(b, c);
		double ca = getScalarProduct(c, a);

		System.out.println("Scalar products: " + ab + ", " + bc + ", " + ca);
		ab /= (getMagnitudeVec3(a) * getMagnitudeVec3(b));
		bc /= (getMagnitudeVec3(b) * getMagnitudeVec3(c));
		ca /= (getMagnitudeVec3(c) * getMagnitudeVec3(a));
		System.out.println("COS T = " + ab + ", " + bc + ", " + ca);
		ab = Math.acos(ab);
		bc = Math.acos(bc);
		ca = Math.acos(ca);
		ab = Math.toDegrees(ab);
		bc = Math.toDegrees(bc);
		ca = Math.toDegrees(ca);

		System.out.println("ANGLES: " + ab + ", " + bc + ", " + ca);
	}

	public static double getScalarProduct(double[] vec3, double[] vec32) {
		return vec3[0] * vec32[0] + vec3[1] * vec32[1] + vec3[2] * vec32[2];
	}

	public static double getScalarProduct(double x1, double y1, double z1, double x2, double y2, double z2) {
		return x1 * x2 + y1 * y2 + z1 * z2;
	}

	public static double getMagnitudeVec3(double[] vec3) {
		return (double) Math.sqrt(vec3[0] * vec3[0] + vec3[1] * vec3[1] + vec3[2] * vec3[2]);
	}

	public static double[] genVec(String name) throws IOException {
		System.out.println("Input vector " + name + ":");
		String[] dat = reader.readLine().split(" ");
		return new double[] {
				Double.parseDouble(dat[0]),
				Double.parseDouble(dat[1]),
				Double.parseDouble(dat[2])
		};
	}
}
