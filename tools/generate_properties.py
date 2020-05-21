#!/usr/bin/env python3 

import re

prop = []
rule = []
table = []

for x in range(0x40000):
    prop.append('XX')

with open('LineBreak.txt', 'r') as file:
    line = file.readline()
    while line:
        line = line.strip()
        if not line.startswith('#'):
            m = re.search("([0-9A-F]{1,6})\.\.([0-9A-F]{1,6})\;([0-9A-Z]{2,})", line)
            if m:
                if int(m.group(2), 16) >= 0x40000:
                    break
                length = int(m.group(2), 16) - int(m.group(1), 16) + 1
                s = int(m.group(1), 16)
                for x in range(length):
                    prop[s + x] = m.group(3)
            else:
                m = re.search("([0-9A-F]{1,6})\;([0-9A-Z]{2,})", line)
                if m:
                    if int(m.group(1), 16) >= 0x40000:
                        break
                    prop[int(m.group(1), 16)] = m.group(2)
        line = file.readline()

prop_type = sorted([x for x in set(prop)])
count = 1
for i in prop_type:
    for j in prop_type:
        # LB2
        # LB3
        # LB4 Always break after hard line breaks.
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
        if j in ("SP", "ZW"):
            rule.append("x")
            continue

        # LB8
        if i == "ZW" and j == "SP":
            rule.append("LB8")
            continue

        # LB8a
        if i == "ZWJ":
            rule.append("x")
            continue

        # LB9

        # LB10

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
        if i not in ("SP", "BA", "HY") and j == "GL":
            rule.append("x")
            continue

        # LB13
        if j in ("CL", "CP", "EX", "IS", "SY"):
            rule.append("x")
            continue

        # LB14
        if i == "OP" and j == "SP":
            rule.append("LB14")
            continue

        # LB15
        if i == "QU" and j == "SP":
            rule.append("LB15")
            continue

        # LB 16
        if i in ("CL", "CP") and j == "NS":
            rule.append("x")
            continue
        if i in ("CL", "CP") and j == "SP":
            rule.append("LB16")
            continue

        # LB17
        if i == "B2" and j == "B2":
            rule.append("x")
            continue
        if i == "B2" and j == "SP":
            rule.append("LB17")
            continue

        # LB18
        if i == "SP":
            rule.append("/")
            continue

        # LB19
        if i == "QU" or j == "QU":
            rule.append("x")
            continue

        # LB20
        if i == "CB" or j == "CB":
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
            rule.append("LB21a")
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
        if j in ("AL", "HL") and j == "NU":
            rule.append("x")
            continue

        # LB23a
        if i == "PR" and j in ("ID", "EB", "EM"):
            rule.append("x")
            continue
        if i in ("ID", "EB", "EM") and j == "PR":
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
        if i in ("JV", "H2") and j in ("JL", "JV"):
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

        # LB30
        if i in ("AL", "HL", "NU") and j == "OP":
            rule.append("LB30")
            continue
        # XXX

        # LB30a
        if i == "RI" and j == "RI":
            rule.append("LB30a")
            continue

        # LB30b
        if i == "EB" and j == "EM":
            rule.append("x")
            continue

        rule.append("/")

prop_type = sorted([x for x in set(prop)])
prop_len = len(prop_type)
count = 1
for i in prop_type:
    print ("pub const %s: u8 = %s;" % (i, str(count)))
    count = count + 1

print ("pub const PROP_COUNT: usize = %d;" % (count - 1));
print()

for a in range(256):
    first_value = prop[a * 1024]
    generate_table =False
    for i in range(1024):
      if prop[a * 1024 + i] != first_value:
          generate_table = True
          break

    if not generate_table:
       table.append("UAX14_PROPERTIES_%s" % first_value)
       continue
     
    print ("#[rustfmt::skip]")
    print ("pub const UAX14_PROPERTIES_%s: [u8; 1024] = [" % str(a))
    for i in range(int(1024 / 16)):
        print(" ", end="")
        for j in range(16):
            print(" %s," % prop[a * 1024 + j + i * 16], end="")
        print()
    print ("];")
    table.append("UAX14_PROPERTIES_%s" % str(a))
    print ()

for t in ["ID", "SG", "XX"]:
    print ("#[rustfmt::skip]")
    print ("pub const UAX14_PROPERTIES_%s: [u8; 1024] = [" % t)
    for i in range(int(1024 / 16)):
        print(" ", end="")
        for j in range(16):
            print(" %s," % t, end="")
        print()
    print ("];")
    print ()

print ("pub const UAX14_PROPERTY_TABLE: [&[u8; 1024]; 256] = [")
for i in table:
    print("  &%s," % i)
print ("];")

print ("pub const UAX14_RULE_TABLE: [i8; %d] = [" % len(rule))
count = 0
for i in rule:
    value = -128; # handing state machine
    if i == "x":
        value = -1
    if i == "/":
        value = -2
    if i == "!":
        value = -3
    print(" %s," % str(value), end="")
    count = count + 1
    if count >= prop_len:
        print ()
        count = 0
print ("];")
