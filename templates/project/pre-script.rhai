print("starting templates:");
print("  - `minimal`: nothing is pregenerated except the project skeleton");
print("  - `counter-example`: come with `counter` contract and nextjs frontend");

template = variable::prompt("Which starting template would you like to use?", default_value: "minimal", ["minimal", "counter-example"])

// if template is `coutner-example` them keep the bloated template

if template == "minimal" {
    file::delete("contracts/counter")
    file::delete("frontend")
}