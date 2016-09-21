import os
import sys
import re


'''
<Desc.>  This function creates the mapping of files where the key is the name before a
         period and the value is a list of extensions with the same key.

TODO: <Issues> What are the corner cases?
         1) File starts with a .
         2) The compare extensions contains multiple .s
         3) There are no .s in the file
'''
def create_map(directory):
    dir_list = os.listdir(directory)
    file_map = {}

    for i in range(0, len(dir_list)):
        kv_pair = re.split(r'\.', dir_list[i])

        # Always create a list
        if len(kv_pair) == 2:
            if kv_pair[0] == '':
                if None not in file_map:
                    file_map[None] = []
                file_map[None].append(kv_pair[1])
            else:
                if kv_pair[0] not in file_map:
                    file_map[kv_pair[0]] = []
                file_map[kv_pair[0]].append(kv_pair[1])
        #TODO: Still Unhandled
        elif len(kv_pair) > 2:
            s = kv_pair[0] + '.'

            if kv_pair[0] == '':
                s = '.'

            for j in range(1, len(kv_pair) - 1):
                s += kv_pair[j] + '.'

            if s[0:-1] not in file_map:
                file_map[s[0:-1]] = []

            file_map[s[0:-1]].append(kv_pair[len(kv_pair) - 1])
        elif len(kv_pair) == 1 and kv_pair[0] != '':
            if kv_pair[0] not in file_map:
                file_map[kv_pair[0]] = []
            file_map[kv_pair[0]].append(None)
        else:
            pass

    return file_map

def remove_nonduplicates(cmp_ext, rm_ext, directory):
    file_map = create_map(cmp_ext, rm_ext, directory)

    #TODO: Modify this for correctness and features later
    for k, v in file_map.items():
        rm_flag = False

        if cmp_ext not in v:
            os.remove(str(key + '.' + rm_ext))


def main():
    if len(sys.argv) < 2:
        print('[x] Nothing to do')
        sys.exit(0)
    else:
        recursive = False

        for i in range(1, len(sys.argv)):
            if sys.argv[i] == '-d':
                directory = sys.argv[i+1]
            if sys.argv[i] == '-c':
                cmp_ext = sys.argv[i+1]
            if sys.argv[i] == '-r':
                rm_ext = sys.argv[i+1]
            if sys.argv[i] == '-R':
                recursive = True


    print(sys.argv)

if __name__ == '__main__':
    main()
