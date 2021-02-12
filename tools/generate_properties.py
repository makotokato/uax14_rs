#!/usr/bin/env python3

import re

prop = []
rule = []
table = []
ea_table = []

for x in range(0x20000):
    prop.append('XX')
    ea_table.append('N')

with open('EastAsianWidth.txt', 'r') as file:
    line = file.readline()
    while line:
        line = line.strip()
        if not line.startswith('#'):
            m = re.search("([0-9A-F]{1,6})\.\.([0-9A-F]{1,6})\;([A-Za-z]{1,})",
                          line)
            if m:
                if int(m.group(2), 16) >= 0x20000:
                    break
                length = int(m.group(2), 16) - int(m.group(1), 16) + 1
                s = int(m.group(1), 16)
                for x in range(length):
                    ea_table[s + x] = m.group(3)
            else:
                m = re.search("([0-9A-F]{1,6})\;([A-Za-z]{1,})", line)
                if m:
                    if int(m.group(1), 16) >= 0x20000:
                        break
                    ea_table[int(m.group(1), 16)] = m.group(2)
        line = file.readline()

with open('LineBreak.txt', 'r') as file:
    line = file.readline()
    while line:
        line = line.strip()
        if not line.startswith('#'):
            m = re.search("([0-9A-F]{1,6})\.\.([0-9A-F]{1,6})\;([0-9A-Z]{2,})",
                          line)
            if m:
                if int(m.group(2), 16) >= 0x20000:
                    break
                length = int(m.group(2), 16) - int(m.group(1), 16) + 1
                s = int(m.group(1), 16)
                for x in range(length):
                    if m.group(3) == "OP":
                        if ea_table[s + x] in ("F", "W", "H"):
                            prop[s + x] = "OP_EA"
                        else:
                            prop[s + x] = "OP_OP30"
                    else:
                        prop[s + x] = m.group(3)
            else:
                m = re.search("([0-9A-F]{1,6})\;([0-9A-Z]{2,})", line)
                if m:
                    if int(m.group(1), 16) >= 0x20000:
                        break
                    # for LB30
                    if m.group(2) == "OP":
                        if ea_table[int(m.group(1), 16)] in ("F", "W", "H"):
                            prop[int(m.group(1), 16)] = "OP_EA"
                        else:
                            prop[int(m.group(1), 16)] = "OP_OP30"
                    elif m.group(2) == "CP" and ea_table[int(
                            m.group(1), 16)] in ("F", "W", "H"):
                        prop[int(m.group(1), 16)] = "CP_EA"
                    else:
                        prop[int(m.group(1), 16)] = m.group(2)
        line = file.readline()

#prop_type = sorted([x for x in set(prop)])
prop_type = sorted([x for x in set(prop)])
prop_type.append("B2_SP")
prop_type.append("CL_CP_SP")
prop_type.append("HL_HY")
prop_type.append("QU_SP")
prop_type.append("RI_RI")
for i in prop_type:
    back_i = i
    for j in prop_type:
        i = back_i

        # AI
        if i == "AI":
            i = "AL"
        if j == "AI":
            j = "AL"

        # break-strict -> NS, others -> ID
        if i == "CJ":
            i = "NS"
        if j == "CJ":
            j = "NS"

        # SA
        if i == "SA":
            i = "AL"
        if j == "SA":
            j = "AL"

        # LB1
        if i == "XX":
            i = "AL"
        if j == "XX":
            j = "AL"

        # LB2
        # LB3
        # LB4
        if i == "BK":
            rule.append("!")
            continue

        # LB5
        if i == "CR" and j == "LF":
            rule.append("x")
            continue
        if i in ("CR", "LF", "NL"):
            rule.append("!")
            continue

        # LB6
        if j in ("BK", "CR", "LF", "NL"):
            rule.append("x")
            continue

        # LB7
        if j == "SP":
            # (LB8)
            if i == "ZW":
                rule.append(i)
                continue
            # (LB14)
            if i in ("OP_OP30", "OP_EA"):
                rule.append("OP_OP30")
                continue
            # (LB15)
            if i in ("QU", "QU_SP"):
                rule.append("QU_SP")
                continue
            # (LB16)
            if i in ("CL", "CP", "CP_EA", "CL_CP_SP"):
                rule.append("CL_CP_SP")
                continue
            # (LB17)
            if i in ("B2", "B2_SP"):
                rule.append("B2_SP")
                continue

        if j in ("SP", "ZW"):
            rule.append("x")
            continue

        # LB8
        if i in ("ZW"):
            rule.append("/")
            continue

        # LB8a
        if i == "ZWJ":
            rule.append("x")
            continue

        # LB9
        if i not in ("BK", "CR", "LF", "NL", "SP", "ZW", "B2_SP", "QU_SP",
                     "CL_CP_SP") and j == "CM":
            rule.append(i)
            continue
        if i not in ("BK", "CR", "LF", "NL", "SP", "ZW", "B2_SP", "QU_SP",
                     "CL_CP_SP") and j == "ZWJ":
            rule.append(i)
            continue

        # LB10
        if i == "CM":
            i = "AL"
        if j == "CM":
            j = "AL"
        if j == "ZWJ":
            j = "AL"

        # LB11
        if i == "WJ":
            rule.append("x")
            continue
        if j == "WJ":
            rule.append("x")
            continue

        # LB12
        if i == "GL":
            rule.append("x")
            continue

        # LB12a
        if i not in ("SP", "BA", "HY", "B2_SP", "QU_SP",
                     "CL_CP_SP") and j == "GL":
            rule.append("x")
            continue

        # LB13
        if j in ("CL", "CP", "CP_EA", "EX", "IS", "SY"):
            rule.append("x")
            continue

        # LB14
        if i in ("OP_OP30", "OP_EA"):
            rule.append("x")
            continue

        # LB15
        if i in ("QU", "QU_SP") and j in ("OP_OP30", "OP_EA"):
            rule.append("x")
            continue
        if i == "QU_SP":
            i = "SP"

        # LB 16
        if i in ("CL", "CP", "CP_EA", "CL_CP_SP") and j == "NS":
            rule.append("x")
            continue
        if i == "CL_CP_SP":
            i = "SP"

        # LB17
        if i in ("B2", "B2_SP") and j == "B2":
            rule.append("x")
            continue
        if i == "B2_SP":
            i = "SP"

        # LB18
        if i == "SP":
            rule.append("/")
            continue

        # LB19
        if i == "QU":
            rule.append("x")
            continue
        if j == "QU":
            rule.append("x")
            continue

        # LB20
        if i == "CB":
            rule.append("/")
            continue
        if j == "CB":
            rule.append("/")
            continue

        # LB21
        # (LB21a)
        if i == "HL" and j in ("HY", "BA"):
            rule.append("HL_HY")
            continue
        if j in ("BA", "HY", "NS"):
            rule.append("x")
            continue
        if i == "BB":
            rule.append("x")
            continue

        # LB21a
        if i == "HL_HY":
            rule.append("x")
            continue

        # LB21b
        if i == "SY" and j == "HL":
            rule.append("x")
            continue

        # LB22
        if j == "IN":
            rule.append("x")
            continue

        # LB23
        if i in ("AL", "HL") and j == "NU":
            rule.append("x")
            continue
        if i == "NU" and j in ("AL", "HL"):
            rule.append("x")
            continue

        # LB23a
        if i == "PR" and j in ("ID", "EB", "EM"):
            rule.append("x")
            continue
        if i in ("ID", "EB", "EM") and j == "PO":
            rule.append("x")
            continue

        # LB24
        if i in ("PR", "PO") and j in ("AL", "HL"):
            rule.append("x")
            continue
        if i in ("AL", "HL") and j in ("PR", "PO"):
            rule.append("x")
            continue

        # LB25
        if i == "NU" and j == "PO":
            rule.append("x")
            continue
        if i == "NU" and j == "PR":
            rule.append("x")
            continue
        if i == "PO" and j == "NU":
            rule.append("x")
            continue
        if i == "PR" and j == "NU":
            rule.append("x")
            continue
        if i == "HY" and j == "NU":
            rule.append("x")
            continue
        if i == "NU" and j == "NU":
            rule.append("x")
            continue

        # LB26
        if i == "JL" and j in ("JL", "JV", "H2", "H3"):
            rule.append("x")
            continue
        if i in ("JV", "H2") and j in ("JV", "JT"):
            rule.append("x")
            continue
        if i in ("JT", "H3") and j == "JT":
            rule.append("x")
            continue

        # LB27
        if i in ("JL", "JV", "JT", "H2", "H3") and j == "IN":
            rule.append("x")
            continue
        if i in ("JL", "JV", "JT", "H2", "H3") and j == "PO":
            rule.append("x")
            continue
        if i == "PR" and j in ("JL", "JV", "JT", "H2", "H3"):
            rule.append("x")
            continue

        # LB28
        if i in ("AL", "HL") and j in ("AL", "HL"):
            rule.append("x")
            continue

        # LB29
        if i == "IS" and j in ("AL", "HL"):
            rule.append("x")
            continue

        # XXX LB30
        if i in ("AL", "HL", "NU") and j == "OP_OP30":
            rule.append("x")
            continue
        if i == "CP" and j in ("AL", "HL", "NU"):
            rule.append("x")
            continue

        # LB30a
        if i == "RI" and j == "RI":
            rule.append("RI_RI")
            continue
        if i == "RI_RI" and j == "RI":
            rule.append("/")
            continue
        if i == "RI_RI":
            i = "RI"

        # LB30b
        if i == "EB" and j == "EM":
            rule.append("x")
            continue

        rule.append("/")

with open('lb_define.rs', 'w') as prop_file:
    #prop_type = sorted([x for x in set(prop)])
    prop_len = len(prop_type)
    count = 1
    for i in prop_type:
        prop_file.write("pub const %s: u8 = %s;\n" % (i, str(count)))
        count = count + 1

    prop_file.write("pub const PROP_COUNT: usize = %d;\n" % (count - 1))
    prop_file.write("\n")

    prop_file.write("#[allow(dead_code)]\n")
    prop_file.write("pub const BREAK_RULE: i8 = -128;\n")
    prop_file.write("pub const KEEP_RULE: i8 = -1;\n")
    prop_file.write("\n")

with open('properties_defines.rs', 'w') as properties_file:
    properties_file.write("use crate::lb_define::*;\n\n")

    for a in range(128):
        first_value = prop[a * 1024]
        generate_table = False
        for i in range(1024):
            if prop[a * 1024 + i] != first_value:
                generate_table = True
                break

        if not generate_table:
            table.append("UAX14_PROPERTIES_%s" % first_value)
            continue

        properties_file.write("pub const UAX14_PROPERTIES_%s: [u8; 1024] = [\n" % str(a))
        for i in range(int(1024)):
            properties_file.write(" %s," % prop[a * 1024 + i])
        properties_file.write("];\n\n")

        table.append("UAX14_PROPERTIES_%s" % str(a))

with open('properties_other.rs', 'w') as properties_file:
    properties_file.write("use crate::lb_define::*;\n\n")

    for t in ["ID", "SG", "XX"]:
        properties_file.write("pub const UAX14_PROPERTIES_%s: [u8; 1024] = [\n" % t)
        for i in range(int(1024 / 16)):
            properties_file.write(" ")
            for j in range(16):
                properties_file.write(" %s," % t)
            properties_file.write("\n");
        properties_file.write("];\n\n");

with open('property_table.rs', 'w') as table_file:
    table_file.write("use crate::properties_defines::*;\n");
    table_file.write("use crate::properties_other::*;\n\n");

    table_file.write("pub const UAX14_PROPERTY_TABLE: [&[u8; 1024]; 128] = [\n")
    for i in table:
        table_file.write("  &%s,\n" % i)
    table_file.write("];\n")

with open('rule_table.rs', 'w') as table_file:
    table_file.write("use crate::lb_define::*;\n\n")
    table_file.write("pub const UAX14_RULE_TABLE: [i8; %d] = [\n" % len(rule))
    count = 0
    line = 0
    table_file.write("// %s\n" % prop_type[line])
    for i in rule:
        value = 0
        # handing state machine
        if i == "x":
            value = -1
        elif i == "/":
            value = -128
        elif i == "!":
            value = -128
        else:
            value = "%s as i8" % i
        table_file.write(" %s," % str(value))
        count = count + 1
        if count >= len(prop_type):
            count = 0
            line = line + 1
            try:
                table_file.write("\n// %s\n" % prop_type[line])
            except:
                pass
    table_file.write("];\n")
