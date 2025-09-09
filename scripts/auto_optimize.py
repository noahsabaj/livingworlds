#!/usr/bin/env python3
"""
Living Worlds Auto-Optimization Pipeline
Analyzes code and automatically suggests performance improvements
"""

import re
import subprocess
from pathlib import Path

class AutoOptimizer:
    def __init__(self):
        self.optimizations = []
        
    def find_o_n2_patterns(self):
        """Find potential O(n²) patterns in Rust code"""
        patterns = [
            # More specific patterns to reduce false positives
            (r'for\s+\w+\s+in\s+.+\.iter\(\)[^}]*for\s+\w+\s+in\s+.+\.iter\(\)', 'Nested iteration over same/similar collections'),
            (r'\.iter\(\)\.find\(\|\w+\|\s*\w+\.\w+\s*==', 'Linear search by field - consider HashMap'),
            (r'provinces\.iter\(\).*provinces\.iter\(\)', 'Double iteration over provinces'),
        ]
        
        for rust_file in Path('src').rglob('*.rs'):
            content = rust_file.read_text()
            for pattern, message in patterns:
                if re.search(pattern, content):
                    self.optimizations.append({
                        'file': str(rust_file),
                        'pattern': pattern,
                        'message': message,
                        'severity': 'high'
                    })
    
    def check_build_times(self):
        """Check if build times are reasonable"""
        import time
        start = time.time()
        result = subprocess.run(
            ['cargo', 'build', '--features', 'bevy/dynamic_linking'],
            capture_output=True,
            text=True
        )
        build_time = time.time() - start
        
        if build_time > 30:  # More than 30 seconds
            self.optimizations.append({
                'file': 'Cargo.toml',
                'message': f'Build time is {build_time:.1f}s - consider enabling more optimizations',
                'severity': 'medium'
            })
    
    def suggest_hashmap_conversions(self):
        """Find places where HashMap would be better"""
        for rust_file in Path('src').rglob('*.rs'):
            content = rust_file.read_text()
            
            # Find .iter().find() patterns
            matches = re.finditer(r'(\w+)\.iter\(\)\.find\(\|(\w+)\| (\w+)\.(\w+) == (\w+)\)', content)
            
            for match in matches:
                collection = match.group(1)
                item_var = match.group(2)
                field = match.group(4)
                search_val = match.group(5)
                
                suggestion = f"""
// BEFORE (O(n)):
{match.group(0)}

// SUGGESTED (O(1)):
// Add this to your struct:
{collection}_by_{field}: HashMap<_, usize>

// Then use:
if let Some(&idx) = self.{collection}_by_{field}.get(&{search_val}) {{
    let {item_var} = &self.{collection}[idx];
}}
"""
                self.optimizations.append({
                    'file': str(rust_file),
                    'line': content[:match.start()].count('\n') + 1,
                    'suggestion': suggestion,
                    'severity': 'high'
                })
    
    def generate_pr(self):
        """Create a GitHub PR with optimizations"""
        if not self.optimizations:
            print("✅ No optimizations needed!")
            return
            
        # Generate markdown report
        report = "# 🚀 Auto-Detected Performance Optimizations\n\n"
        
        high_priority = [o for o in self.optimizations if o['severity'] == 'high']
        medium_priority = [o for o in self.optimizations if o['severity'] == 'medium']
        
        if high_priority:
            report += "## 🔴 High Priority\n\n"
            for opt in high_priority:
                report += f"- **{opt['file']}**: {opt['message']}\n"
                if 'suggestion' in opt:
                    report += f"```rust\n{opt['suggestion']}\n```\n"
        
        if medium_priority:
            report += "\n## 🟡 Medium Priority\n\n"
            for opt in medium_priority:
                report += f"- {opt.get('function', opt['file'])}: {opt['message']}\n"
        
        # Save report
        Path('OPTIMIZATION_REPORT.md').write_text(report)
        
        # Check if we're already on the optimization branch
        current_branch = subprocess.run(
            ['git', 'branch', '--show-current'],
            capture_output=True,
            text=True
        ).stdout.strip()
        
        if current_branch != 'auto/performance-optimizations':
            # Try to create branch, or switch if it exists
            result = subprocess.run(
                ['git', 'checkout', '-b', 'auto/performance-optimizations'],
                capture_output=True,
                text=True
            )
            if result.returncode != 0:
                subprocess.run(['git', 'checkout', 'auto/performance-optimizations'])
        
        subprocess.run(['git', 'add', 'OPTIMIZATION_REPORT.md'])
        subprocess.run(['git', 'commit', '-m', 'perf: Auto-detected optimization opportunities'])
        
        print(f"📊 Found {len(self.optimizations)} optimization opportunities!")
        print(f"   High priority: {len(high_priority)}")
        print(f"   Medium priority: {len(medium_priority)}")

if __name__ == "__main__":
    import sys
    
    optimizer = AutoOptimizer()
    
    # Run different checks based on arguments
    if '--quick' in sys.argv:
        print("Running quick checks...")
        optimizer.find_o_n2_patterns()
        optimizer.suggest_hashmap_conversions()
    else:
        print("Running full analysis...")
        optimizer.find_o_n2_patterns()
        optimizer.suggest_hashmap_conversions()
        optimizer.check_build_times()
    
    # Always generate report
    optimizer.generate_pr()
    
    # Exit with error code if high priority issues found
    high_priority = [o for o in optimizer.optimizations if o['severity'] == 'high']
    if high_priority:
        sys.exit(1)  # Can be used in CI/CD to fail builds