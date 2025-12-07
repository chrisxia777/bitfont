# Java

* See [BitFontData.java] for a demo of using the fontdata.

## Running

To compile and run this code with [Maven]:

```text
mvn install
mvn compile
mvn exec:java
```

## Using Eclipse

To open the project in Eclipse:

* Navigate to: `File > Import > Maven > Existing Maven Project`
* For "Root Directory" select the `bitfont/java` folder
* Make sure the project is selected and click "Finish"

After a few moments, the project will appear in the navigator under "nato".

## Using IntelliJ IDEA

To open the project in IntelliJ IDEA:

* Ensure the bundled "Maven Integration" plugin is enabled.
* Select `File > New > Project from Existing Sources...` from the menu.
* Navigate in the file selector to this `java` subdirectory.
* Ensure "Import project from external model" and "Maven" are selected, click Next.
* You can accept the defaults on the import project dialog. Click Next.
* Leave the profile unselected. Click Next.
* The `com.stripe.interview:nato:1.0-SNAPSHOT` project should be selected. If not, select it. Click Next.
* Set Project SDK to JDK 1.8. Click Next.
* Pick a name for the project or just accept the default. Click Finish.

A new window should then appear with the project.

[BitFontData.java]: ./src/main/java/com/stripe/interview/BitFontData.java
[Maven]: https://maven.apache.org/
