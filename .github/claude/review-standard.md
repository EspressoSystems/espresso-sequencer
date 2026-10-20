Perform a comprehensive code review with the following focus areas.

How to look for bugs, whatever the focus area:

- For every deleted or replaced line, name the behavior it enforced and find where the new code re-establishes it. If
  you can't find it, that's a finding.
- Give every finding a concrete failure scenario: the input or state, and the wrong result it produces.
- If you find a plausible behavior change, report it as a finding. Don't downgrade it to a question.

1. **Code Quality**
   - Clean code principles and best practices
   - Proper error handling and edge cases
   - Code readability and maintainability

2. **Security**
   - Check for potential security vulnerabilities
   - Validate input sanitization
   - Review authentication/authorization logic
   - Flag credentials (RPC URLs with API keys, private keys, tokens) that can reach logs, error messages, metrics,
     telemetry or API responses

3. **Performance**
   - Identify potential performance bottlenecks
   - Review database queries for efficiency
   - Check for memory leaks or resource issues

4. **Testing**
   - Verify adequate test coverage
   - Review test quality and edge cases
   - Check for missing test scenarios

5. **Documentation**
   - Ensure code is properly documented
   - Verify README updates for new features
   - Check API documentation accuracy

6. **Operator notes**
   - If the change requires operator intervention and adds no `doc/operator-notes/*.md` fragment, flag it and suggest
     the fragment. Otherwise stay silent.
