class {{ classname }} < Formula
  desc "{{ project.description }}"
  {% if project.homepage -%}
    homepage "{{ project.homepage }}"
  {% elif project.repository -%}
    homepage "{{ project.repository }}"
  {% else -%}
    homepage ""
  {% endif -%}
  url "{{ url }}"
  sha256 "{{ checksum }}"
  license "{{ project.license }}"

  depends_on "shinc"

  def install
    bin.install Dir["bin/*"]
    # Install man pages
    man.install Dir["share/man/*"]
    # Install bash, fish, and zsh completions
    bash_completion.install Dir["share/completions/bash/*"]
    fish_completion.install Dir["share/completions/fish/*"]
    zsh_completion.install Dir["share/completions/zsh/*"]
  end

  test do
    {% for bin in bins -%}
      system "#{bin}/{{ bin.name }}", "--version"
    {%- endfor %}
  end
end
