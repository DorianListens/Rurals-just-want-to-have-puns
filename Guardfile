# A sample Guardfile
# More info at https://github.com/guard/guard#readme

# Uncomment and set this to only include directories you want to watch
#
# directories %(app lib config test spec feature)

guard :shell do
  watch %r{\.rs$} do
    `cargo run`
  end
end

