module jp.cafebabe.btmeister {
    requires com.fasterxml.jackson.databind;
    requires io.vavr;
    requires info.picocli;
    requires jp.cafebabe.diranger;
    requires org.slf4j;
    requires java.logging;

    opens jp.cafebabe.btmeister.cli to info.picocli;

    exports jp.cafebabe.btmeister;
    exports jp.cafebabe.btmeister.matchers;
    exports jp.cafebabe.btmeister.utils;
    opens jp.cafebabe.btmeister.cli.io to info.picocli;
}
