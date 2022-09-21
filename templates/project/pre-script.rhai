let template = variable::prompt("Which starting template would you like to use?", "minimal", ["minimal", "counter-example"]);

// if template is `coutner-example` them keep the bloated template
if template == "minimal" {
    file::delete("contracts/counter");
    file::delete("frontend");
}