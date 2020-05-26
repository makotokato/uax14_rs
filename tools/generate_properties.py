#!/usr/bin/env python3 

import re

prop = []
rule = []
table = []

for x in range(0x20000):
    prop.append('XX')

with open('LineBreak.txt', 'r') as file:
    line = file.readline()
    while line:
        line = line.strip()
        if not line.startswith('#'):
            m = re.search("([0-9A-F]{1,6})\.\.([0-9A-F]{1,6})\;([0-9A-Z]{2,})", line)
            if m:
                if int(m.group(2), 16) >= 0x20000:
                    break
                length = int(m.group(2), 16) - int(m.group(1), 16) + 1
                s = int(m.group(1), 16)
                for x in range(length):
                    prop[s + x] = m.group(3)
            else:
                m = re.search("([0-9A-F]{1,6})\;([0-9A-Z]{2,})", line)
                if m:
                    if int(m.group(1), 16) >= 0x20000:
                        break
                    prop[int(m.group(1), 16)] = m.group(2)
        line = file.readline()

#prop_type = sorted([x for x in set(prop)])
prop_type = sorted([x for x in set(prop)])
prop_type.append("B2_SP")
prop_type.append("CL_CP_SP")
prop_type.append("HL_HY")
prop_type.append("QU_SP")
for i in prop_type:
    back_i = i;
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
        if i in("CR", "LF", "NL"):
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
            if i == "OP":
                rule.append(i)
                continue
            # (LB15)
            if i in ("QU", "QU_SP"):
                rule.append("QU_SP")
                continue
            # (LB16)
            if i in ("CL", "CP", "CL_CP_SP"):
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
        if i not in ("BK", "CR", "LF", "NL", "SP", "ZW", "B2_SP", "QU_SP", "CL_CP_SP") and j == "CM":
            rule.append(i)
            continue
        if i not in ("BK", "CR", "LF", "NL", "SP", "ZW", "B2_SP", "QU_SP", "CL_CP_SP") and j == "ZWJ":
            rule.append(i)
            continue

        # LB10
        if i == "CM":
            i = "AL"
        if i == "ZWJ":
            i = "AL"

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
        if i not in ("SP", "BA", "HY", "B2_SP", "QU_SP", "CL_CP_SP") and j == "GL":
            rule.append("x")
            continue

        # LB13
        if j in ("CL", "CP", "EX", "IS", "SY"):
            rule.append("x")
            continue

        # LB14
        if i in ("OP"):
            rule.append("x")
            continue

        # LB15
        if i in ("QU", "QU_SP") and j == "OP":
            rule.append("x")
            continue
        if i == "QU_SP":
            i = "SP"

        # LB 16
        if i in ("CL", "CP", "CL_CP_SP") and j == "NS":
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
        if j in ("BA", "HY", "NS"):
            rule.append("x")
            continue
        if i == "BB":
            rule.append("x")
            continue

        # LB21a
        if i == "HL" and j in ("HY", "BA"):
            rule.append("HL_HY")
            continue
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
        if i == "CL" and j == "PO":
            rule.append("x")
            continue
        if i == "CP" and j == "PO":
            rule.append("x")
            continue
        if i == "CL" and j == "PR":
            rule.append("x")
            continue
        if i == "CP" and j == "PR":
            rule.append("x")
            continue
        if i == "NU" and j == "PO":
            rule.append("x")
            continue
        if i == "NU" and j == "PR":
            rule.append("x")
            continue
        if i == "PO" and j == "OP":
            rule.append("x")
            continue
        if i == "PO" and j == "NU":
            rule.append("x")
            continue
        if i == "PR" and j == "OP":
            rule.append("x")
            continue
        if i == "PR" and j == "NU":
            rule.append("x")
            continue
        if i == "HY" and j == "NU":
            rule.append("x")
            continue
        if i == "IS" and j == "NU":
            rule.append("x")
            continue
        if i == "NU" and j == "NU":
            rule.append("x")
            continue
        if i == "SY" and j == "NU":
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
        if i in ("AL", "HL", "NU") and j == "OP":
            rule.append("x")
            continue
        if j in ("AL", "HL", "NU") and i == "CP":
            rule.append("x")
            continue

        # LB30a

        # LB30b
        if i == "EB" and j == "EM":
            rule.append("x")
            continue

        rule.append("/")

#prop_type = sorted([x for x in set(prop)])
prop_len = len(prop_type)
count = 1
for i in prop_type:
    print ("pub const %s: u8 = %s;" % (i, str(count)))
    count = count + 1

print ("pub const PROP_COUNT: usize = %d;" % (count - 1));
print()

for a in range(128):
    first_value = prop[a * 1024]
    generate_table =False
    for i in range(1024):
      if prop[a * 1024 + i] != first_value:
          generate_table = True
          break

    if not generate_table:
       table.append("UAX14_PROPERTIES_%s" % first_value)
       continue
     
    print ("pub const UAX14_PROPERTIES_%s: [u8; 1024] = [" % str(a))
    for i in range(int(1024)):
        print(" %s," % prop[a * 1024 + i], end="")
    print ("];")
    table.append("UAX14_PROPERTIES_%s" % str(a))
    print ()

for t in ["ID", "SG", "XX"]:
    print ("pub const UAX14_PROPERTIES_%s: [u8; 1024] = [" % t)
    for i in range(int(1024 / 16)):
        print(" ", end="")
        for j in range(16):
            print(" %s," % t, end="")
        print()
    print ("];")
    print ()

print ("pub const UAX14_PROPERTY_TABLE: [&[u8; 1024]; 128] = [")
for i in table:
    print("  &%s," % i)
print ("];")

print ("pub const UAX14_RULE_TABLE: [i8; %d] = [" % len(rule))
count = 0
line = 0
print ("// %s" % prop_type[line])
for i in rule:
    value = 0; # handing state machine
    if i == "x":
        value = -1
    elif i == "/":
        value = -128
    elif i == "!":
        value = -128
    else:
        value = "%s as i8" % i
    print(" %s /* %s */," % (str(value), prop_type[count]), end="")
    count = count + 1
    if count >= len(prop_type):
        print ()
        count = 0
        line = line + 1
        try:
             print ("// %s" % prop_type[line])
        except:
             pass
print ("];")
