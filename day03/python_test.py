def get_most_common_value_at_index(values_list, index):
    zero = 0
    one = 0

    for value in values_list:
        if value[index] == "0":
            zero += 1
        elif value[index] == "1":
            one += 1
        else:
            raise Exception("Non '0' or '1' character in value")

    return "0" if zero > one else "1"


def invert_bit(value):
    if value == "0":
        return "1"
    elif value == "1":
        return "0"
    else:
        raise Exception("Non '0' or '1' character in value")


if __name__ == "__main__":
    with open("input.txt", "r") as input_file:
        values = input_file.read().split()

    gamma_rate = ""
    epsilon_rate = ""

    for location in range(12):
        most_common = get_most_common_value_at_index(values, location)
        gamma_rate += most_common
        epsilon_rate += invert_bit(most_common)

    print(f"Part 1: {int(gamma_rate, 2) * int(epsilon_rate, 2)}")


#     PART 2
    oxy_gen_values = values[:]
    c02_values = values[:]

    for i in range(12):

        if len(oxy_gen_values) > 1:
            most_common_bit = get_most_common_value_at_index(oxy_gen_values, i)
            oxy_gen_values = list(filter(lambda x: x[i] == most_common_bit, oxy_gen_values))

        if len(c02_values) > 1:
            most_common_bit = get_most_common_value_at_index(c02_values, i)
            least_common_bit = invert_bit(most_common_bit)
            most_common_bit = get_most_common_value_at_index(c02_values, i)
            c02_values = list(filter(lambda x: x[i] == least_common_bit, c02_values))

    print(f"Oxygen Generator Value {int(oxy_gen_values[0], 2)}")
    print(f"C02 Scrubber Value {int(c02_values[0], 2)}")

    print(f"Part 2: {int(oxy_gen_values[0], 2) * int(c02_values[0], 2)}")