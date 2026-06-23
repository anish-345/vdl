#!/usr/bin/env python3
"""
VDL - Video Downloader Library CLI Wrapper

Cross-platform video downloader with extensible architecture.
"""

import subprocess
import sys
import argparse
import os
import shutil


def check_yt_dlp():
    """Check if yt-dlp is available"""
    return shutil.which('yt-dlp') is not None


def run_backend(args, backend='yt-dlp'):
    """Run the specified backend"""
    cmd = [backend] + args
    try:
        result = subprocess.run(cmd, check=True)
        return result.returncode
    except subprocess.CalledProcessError as e:
        print(f"Error: {e}")
        return e.returncode
    except FileNotFoundError:
        print(f"Error: {backend} not found.")
        if backend == 'yt-dlp':
            print("Install with: pip install yt-dlp")
        return 1


def main():
    parser = argparse.ArgumentParser(
        description='Video Downloader Library - Cross-platform video downloader',
        formatter_class=argparse.RawDescriptionHelpFormatter,
        epilog="""
Examples:
  vdl-wrapper.py https://example.com/video
  vdl-wrapper.py https://example.com/video -o output.mp4
  vdl-wrapper.py https://example.com/video --formats
        """
    )
    
    parser.add_argument('url', nargs='?', help='Video URL to download')
    parser.add_argument('-o', '--output', help='Output filename')
    parser.add_argument('-f', '--format', help='Format selection')
    parser.add_argument('-q', '--quality', help='Quality (e.g., 1080, 720)')
    parser.add_argument('--formats', action='store_true', help='List available formats')
    parser.add_argument('--quiet', action='store_true', help='Silent mode')
    parser.add_argument('--backend', choices=['yt-dlp', 'vdl'], default=None,
                        help='Backend to use (auto-detects yt-dlp first)')
    
    args = parser.parse_args()
    
    if not args.url:
        parser.print_help()
        return 0
    
    cmd_args = [args.url]
    
    if args.output:
        cmd_args.extend(['-o', args.output])
    if args.format:
        cmd_args.extend(['-f', args.format])
    if args.quality:
        cmd_args.extend(['-f', f'best[height<={args.quality}]'])
    if args.formats:
        cmd_args.append('--list-formats')
    if args.quiet:
        cmd_args.append('--quiet')
    
    backend = args.backend
    if backend is None:
        backend = 'yt-dlp' if check_yt_dlp() else 'vdl'
    
    return run_backend(cmd_args, backend)


if __name__ == '__main__':
    sys.exit(main())