import os
import sys
import re


'''
<Desc.>  This function creates the mapping of files where the key is the name before a
         period and the value is a list of extensions with the same key.

TODO:
<Issues> What are the corner cases?
         1) File starts with a .
         2) The compare extensions contains multiple .s
         3) There are no .s in the file
'''
def create_map(directory):
    dir_list = os.listdir(directory)
    file_map = {}

    for i in range(0, len(dir_list)):
        kv_pair = re.split(r'\.', dir_list[i])

        if len(kv_pair) > 2:
            pass
        else:
            pass

def remove_duplicates(cmp_ext, rm_ext, directory):
    file_map = create_map(cmp_ext, rm_ext, directory)

    #TODO: Modify this for correctness and features later
    for k, v in file_map.items():
        rm_flag = False

        if cmp_ext not in v:
            os.remove(str(key + '.' + rm_ext))


def main():
    if len(sys.args) < 1:
        pass
    else:
        pass

if __name__ == '__main__':
    main()
