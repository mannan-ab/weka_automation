import weka.classifiers.Evaluation;
import weka.classifiers.bayes.NaiveBayesSimple;
import weka.classifiers.trees.J48;
import weka.core.Instance;
import weka.core.Instances;
import weka.core.converters.ConverterUtils.DataSource;

import java.util.Random;

public class Experimenter {
    public static void main(String[] args) throws Exception {
        // Load datasets
        Instances trainData = DataSource.read("agaricus-lepiota.trainingdata_converted_noisy (1).arff");
        Instances testData = DataSource.read("cagaricus-lepiota.testdata_converted_noisy (1).arff");
        trainData.setClassIndex(trainData.numAttributes() - 1);
        testData.setClassIndex(testData.numAttributes() - 1);

        
        double[] confidenceFactors = {0.1, 0.25, 0.5};
        int[] minNumObjects = {2, 5, 10};
        double bestError = Double.MAX_VALUE;
        String bestModelDesc = "";
        J48 bestJ48 = null;
        NaiveBayesSimple bestNB = null;

        // Evaluate J48 with different parameters
        System.out.println("=== J48 Cross-Validation Errors ===");
        for (double c : confidenceFactors) {
            for (int m : minNumObjects) {
                J48 j48 = new J48();
                j48.setConfidenceFactor((float) c);
                j48.setMinNumObj(m);

                Evaluation eval = new Evaluation(trainData);
                eval.crossValidateModel(j48, trainData, 10, new Random(1));
                double error = eval.pctIncorrect() / 100.0;
                System.out.printf("J48 (C=%.2f, M=%d): %.4f\n", c, m, error);

                if (error < bestError) {
                    bestError = error;
                    bestModelDesc = String.format("J48 (C=%.2f, M=%d)", c, m);
                    bestJ48 = j48;
                    bestNB = null;
                }
            }
        }

        // Evaluate NaiveBayesSimple
        NaiveBayesSimple nb = new NaiveBayesSimple();
        Evaluation nbEval = new Evaluation(trainData);
        nbEval.crossValidateModel(nb, trainData, 10, new Random(1));
        double nbError = nbEval.pctIncorrect() / 100.0;
        System.out.println("\n=== NaiveBayesSimple Cross-Validation Error ===");
        System.out.printf("NaiveBayesSimple: %.4f\n", nbError);

        if (nbError < bestError) {
            bestError = nbError;
            bestModelDesc = "NaiveBayesSimple";
            bestJ48 = null;
            bestNB = nb;
        }

        // Train the best model on the entire training set
        System.out.println("\n=== Best Model ===");
        System.out.println("Best Model: " + bestModelDesc + " (CV Error: " + bestError + ")");
        if (bestJ48 != null) {
            bestJ48.buildClassifier(trainData);
        } else {
            bestNB.buildClassifier(trainData);
        }

        // Classify test instances and compute test error
        System.out.println("\n=== Test Instance Classifications ===");
        int incorrect = 0;
        for (int i = 0; i < testData.numInstances(); i++) {
            Instance instance = testData.instance(i);
            double pred = (bestJ48 != null) ? bestJ48.classifyInstance(instance) : bestNB.classifyInstance(instance);
            String predictedClass = testData.classAttribute().value((int) pred);
            String actualClass = instance.stringValue(testData.classIndex());
            System.out.printf("Instance %d: Predicted=%s, Actual=%s\n", i, predictedClass, actualClass);
            if (pred != instance.classValue()) incorrect++;
        }

        // Compute and output test error
        double testError = (double) incorrect / testData.numInstances();
        System.out.println("\n=== Test Error ===");
        System.out.printf("Test Error: %.4f\n", testError);
    }
}
