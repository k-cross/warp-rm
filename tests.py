import os
import shutil
import sys
import unittest

import warp_rm

class WarpTests(unittest.TestCase):
    def setUp(self):
        self.directory = 'autotests'
        test_filenames = ['t.f', 't', '.t', 't.f.t.f.t', 't - f.ttt.fff']

        if self.directory in os.listdir():
            self.fail('autotests dir exists')

        os.mkdir(self.directory)

        for e in test_filenames:
            with open((self.directory + '/' + e), 'xt') as f:
                f.write('test')
        

    def tearDown(self):
        shutil.rmtree('autotests')

    def test_create_map_io(self):
        warp_map = warp_rm.create_map(self.directory)
        print(warp_map)

        for k, v in warp_map.items():
            self.assertEqual(type(list()), type(v))

            if k == None:
                self.assertIn('t', warp_map[k])
            elif k == 't':
                self.assertIn(None, warp_map[k])
                self.assertIn('f', warp_map[k])
            elif k == 't.f.t.f':
                self.assertIn('t', warp_map[k])
            elif k == 't - f.ttt':
                self.assertIn('fff', warp_map[k])
            else:
                self.fail('Unknown Case')

if __name__ == '__main__':
    unittest.main()
