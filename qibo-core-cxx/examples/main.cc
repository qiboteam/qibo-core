#include "lib.rs.h"

#include <iostream>
#include <vector>
using namespace std;
using namespace qibo_core;

int main() {
    auto c = make_circuit(5);
    vector<size_t> gid = {2};
    c->add(make_x_gate(), gid);
    cout << static_cast<std::string>(c->draw()) << endl;
    return 0;
}
