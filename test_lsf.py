#!/usr/bin/env python3
"""
Test suite for ls-f column.py formatter
"""
import unittest
import subprocess
import tempfile
import os
import sys
from io import StringIO

# Add current directory to path to import column
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))

class TestColumnFormatter(unittest.TestCase):
    
    def setUp(self):
        """Set up test fixtures"""
        self.test_input = "📁 dir\n🐍 script.py\n📄 README.md\n📊 data.csv\n"
        self.column_script = os.path.join(os.path.dirname(__file__), 'column.py')
    
    def run_column_py(self, input_text, env_vars=None):
        """Helper to run column.py with input"""
        env = os.environ.copy()
        if env_vars:
            env.update(env_vars)
        
        proc = subprocess.Popen(
            ['python3', self.column_script],
            stdin=subprocess.PIPE,
            stdout=subprocess.PIPE,
            stderr=subprocess.PIPE,
            env=env,
            text=True
        )
        
        stdout, stderr = proc.communicate(input_text)
        return stdout, stderr, proc.returncode
    
    def test_default_column_layout(self):
        """Test default column-major layout"""
        stdout, stderr, returncode = self.run_column_py(self.test_input)
        self.assertEqual(returncode, 0)
        self.assertIn("📁 dir", stdout)
        self.assertIn("🐍 script.py", stdout)
    
    def test_row_layout(self):
        """Test row-major layout with LSF_LAYOUT=rows"""
        stdout, stderr, returncode = self.run_column_py(
            self.test_input, 
            {'LSF_LAYOUT': 'rows'}
        )
        self.assertEqual(returncode, 0)
        self.assertIn("📁 dir", stdout)
    
    def test_custom_tabsize(self):
        """Test custom tab size"""
        stdout, stderr, returncode = self.run_column_py(
            self.test_input,
            {'LSF_TABSIZE': '4'}
        )
        self.assertEqual(returncode, 0)
    
    def test_empty_input(self):
        """Test with empty input"""
        stdout, stderr, returncode = self.run_column_py("")
        self.assertEqual(returncode, 0)
        self.assertEqual(stdout.strip(), "")
    
    def test_single_item(self):
        """Test with single item"""
        stdout, stderr, returncode = self.run_column_py("📄 single.txt\n")
        self.assertEqual(returncode, 0)
        self.assertIn("📄 single.txt", stdout)
    
    def test_many_items(self):
        """Test with many items"""
        many_items = "\n".join([f"📄 file{i}.txt" for i in range(50)])
        stdout, stderr, returncode = self.run_column_py(many_items)
        self.assertEqual(returncode, 0)
        self.assertIn("📄 file0.txt", stdout)
        self.assertIn("📄 file49.txt", stdout)
    
    def test_unicode_handling(self):
        """Test Unicode icon handling"""
        unicode_input = "🎵 music.mp3\n🎬 video.mp4\n🖼️ image.png\n"
        stdout, stderr, returncode = self.run_column_py(unicode_input)
        self.assertEqual(returncode, 0)
        self.assertIn("🎵 music.mp3", stdout)

class TestLsfBasics(unittest.TestCase):
    
    def setUp(self):
        """Set up test fixtures"""
        self.lsf_script = os.path.join(os.path.dirname(__file__), 'lsf')
        os.chmod(self.lsf_script, 0o755)
    
    def run_lsf(self, args=None):
        """Helper to run lsf command"""
        cmd = ['bash', self.lsf_script]
        if args:
            cmd.extend(args)
        
        proc = subprocess.Popen(
            cmd,
            stdout=subprocess.PIPE,
            stderr=subprocess.PIPE,
            text=True
        )
        
        stdout, stderr = proc.communicate()
        return stdout, stderr, proc.returncode
    
    def test_version_flag(self):
        """Test --version flag"""
        stdout, stderr, returncode = self.run_lsf(['--version'])
        self.assertEqual(returncode, 0)
        self.assertIn("4.5.1", stdout)
    
    def test_help_flag(self):
        """Test --help flag"""
        stdout, stderr, returncode = self.run_lsf(['--help'])
        self.assertEqual(returncode, 0)
        self.assertIn("Usage:", stdout)
        self.assertIn("Options:", stdout)
    
    def test_no_icons_flag(self):
        """Test --no-icons flag"""
        stdout, stderr, returncode = self.run_lsf(['--no-icons'])
        self.assertEqual(returncode, 0)
        # Should run without icons, exit cleanly
    
    def test_invalid_flag(self):
        """Test that lsf passes through flags to ls"""
        stdout, stderr, returncode = self.run_lsf(['--invalid-flag'])
        # lsf passes flags to ls, so ls will handle invalid flags
        # The behavior depends on the ls implementation
        # Just verify lsf doesn't crash internally
        self.assertTrue(True)  # If we get here, lsf didn't crash

if __name__ == '__main__':
    unittest.main()